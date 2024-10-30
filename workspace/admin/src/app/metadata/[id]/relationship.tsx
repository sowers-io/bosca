import { JsonEditor } from '@/components/json-editor'
import React from 'react'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'
import { redirect } from 'next/navigation'
import { ButtonLink } from '@/components/button-link'

const saveRelationshipMutation = gql`
  mutation EditRelationship($relationship: MetadataRelationshipInput!) {
    content {
      metadata {
        editRelationship(relationship: $relationship)
      }
    }
  }
`

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function Relationship({ metadata, relationship, index }: { metadata: any, relationship: any, index: number }) {
  async function save(formData: FormData) {
    'use server'
    const id1 = formData.get('id1')
    const id2 = formData.get('id2')
    const attributes = JSON.parse(formData.get('attributes')!.toString())
    const variables = {
      relationship: {
        id1: id1,
        id2: id2,
        attributes: attributes,
        relationship: formData.get('relationship'),
      },
    }
    await getClient().mutate({ mutation: saveRelationshipMutation, variables: variables })
    redirect('/metadata/' + id1)
  }
  return (
    <form action={save}>
      <input type="hidden" name="id1" value={metadata.id}/>
      <input type="hidden" name="id2" value={relationship.metadata.id}/>
      <table className={'w-full' + (index === 0 ? '' : ' mt-8')}>
        <tbody>
          <tr>
            <td><a href={'/metadata/' + relationship.metadata.id}>{relationship.metadata.name}</a></td>
            <td className={'text-end'}>
              {relationship.relationship && relationship.relationship.length > 0 ?
                <>{relationship.relationship.trim()}<input type="hidden" name="relationship" value={relationship.relationship} /></> :
                <Input name="relationship" defaultValue={relationship.relationship}/>
              }
            </td>
          </tr>
          <tr>
            <td colSpan={2} className={'pt-4'}>
              <JsonEditor
                id={metadata.id}
                editable={true}
                attributes={relationship.attributes}/>
            </td>
          </tr>
        </tbody>
      </table>
      <div className="inline-flex gap-3 mt-4">
        <Button type="submit" size="sm">Save</Button>
        <ButtonLink
          href={'/metadata/' + metadata.id + '?delete-relationship=' + relationship.metadata.id + '&relationship=' + relationship.relationship}
          variant="outline" size="sm">
          Delete
        </ButtonLink>
      </div>
    </form>
  )
}